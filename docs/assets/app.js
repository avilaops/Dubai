const selectors = (name) => document.querySelector(`[data-field="${name}"]`);

const currency = (value) =>
  typeof value === "number"
    ? value.toLocaleString("pt-PT", {
        style: "currency",
        currency: "EUR",
        minimumFractionDigits: 2,
      })
    : "—";

const number = (value, options = {}) =>
  typeof value === "number"
    ? value.toLocaleString("pt-PT", { maximumFractionDigits: 2, ...options })
    : "—";

const hasNumber = (value) => typeof value === "number" && Number.isFinite(value);

const measurement = (value, unit, options = {}) =>
  hasNumber(value) ? `${number(value, options)} ${unit}` : "—";

const formatDate = (value) => {
  try {
    const date = new Date(value);
    if (Number.isNaN(date.getTime())) {
      return value;
    }
    return new Intl.DateTimeFormat("pt-PT", { dateStyle: "long" }).format(date);
  } catch (error) {
    console.warn("Falha ao formatar data", value, error);
    return value;
  }
};

const formatRange = (start, end) => {
  if (!start || !end) return "—";
  return `${formatDate(start)} — ${formatDate(end)}`;
};

const joinList = (items) => {
  if (!Array.isArray(items) || items.length === 0) return "—";
  if (items.length === 1) return items[0];
  return `${items.slice(0, -1).join(", ")} e ${items.at(-1)}`;
};

function renderInvoice(invoice) {
  selectors("source").textContent = invoice.source ?? "Fonte desconhecida";
  selectors("invoiceNumber").textContent = invoice.invoice_number ?? "Sem número";
  selectors("atcud").textContent = invoice.atcud ?? "ATCUD";
  selectors("issueDate").textContent = formatDate(invoice.issue_date);
  selectors("billingRange").textContent = formatRange(
    invoice.billing_period?.start,
    invoice.billing_period?.end,
  );
  selectors("dueDate").textContent = formatDate(invoice.due_date);
  selectors("totalAmount").textContent = currency(invoice.total_amount_eur);

  selectors("electricityTotal").textContent = currency(
    invoice.breakdown?.electricity_total_eur,
  );
  selectors("electricityNet").textContent = currency(
    invoice.breakdown?.electricity_without_taxes_eur,
  );
  selectors("tariff").textContent = invoice.contract?.tariff ?? "—";
  selectors("power").textContent = measurement(
    invoice.contract?.power_kva,
    "kVA",
    { maximumFractionDigits: 2 },
  );
  selectors("consumptionTotal").textContent = measurement(
    invoice.consumption?.total_kwh,
    "kWh",
  );
  selectors("co2").textContent = measurement(
    invoice.consumption?.co2_emissions_kg,
    "kg",
  );

  selectors("entity").textContent = invoice.payment?.entity ?? "—";
  selectors("reference").textContent = invoice.payment?.reference ?? "—";

  selectors("customerName").textContent = invoice.customer?.name ?? "—";
  selectors("customerNif").textContent = invoice.customer?.nif ?? "—";
  selectors("customerAddress").textContent = invoice.customer?.billing_address ?? "—";

  selectors("contractCode").textContent = invoice.contract?.code ?? "—";
  selectors("contractCpe").textContent = invoice.contract?.cpe ?? "—";

  const previous = invoice.meter_readings?.previous;
  const current = invoice.meter_readings?.current;
  selectors("previousReadings").textContent = previous
    ? `${formatDate(previous.date)} · Vazio ${number(previous.vazio)} · Cheias ${number(
        previous.cheias,
      )} · Ponta ${number(previous.ponta)}`
    : "—";
  selectors("currentReadings").textContent = current
    ? `${formatDate(current.date)} · Vazio ${number(current.vazio)} · Cheias ${number(
        current.cheias,
      )} · Ponta ${number(current.ponta)}`
    : "—";

  const breakdownContainer = selectors("consumptionBreakdown");
  breakdownContainer.innerHTML = "";
  const breakdown = invoice.consumption?.breakdown_kwh ?? {};
  Object.entries(breakdown).forEach(([key, value]) => {
    const chip = document.createElement("span");
    chip.className = "chip";
    chip.textContent = `${key.replace(/_/g, " ")}: ${number(value)} kWh`;
    breakdownContainer.appendChild(chip);
  });
  selectors("consumptionNotes").textContent = joinList(invoice.consumption?.notes);

  const chargesContainer = selectors("charges");
  chargesContainer.innerHTML = "";
  (invoice.charges_without_vat ?? []).forEach((item) => {
    const quantity = item.quantity ?? "—";
    const unit = item.unit ?? "";
    const unitPrice = hasNumber(item.unit_price_eur)
      ? currency(item.unit_price_eur)
      : "—";
    const amount = hasNumber(item.amount_eur)
      ? currency(item.amount_eur)
      : "—";
    const vat = hasNumber(item.vat_rate)
      ? `${number(item.vat_rate * 100, { maximumFractionDigits: 0 })}%`
      : "—";

    const row = document.createElement("div");
    row.className = "table__row";
    row.innerHTML = `
      <span>${item.description}</span>
      <span class="numeric">${quantity} ${unit}</span>
      <span class="numeric">${unitPrice}</span>
      <span class="numeric">${amount}</span>
      <span class="numeric">${vat}</span>
    `;
    chargesContainer.appendChild(row);
  });
  if (!chargesContainer.children.length) {
    const row = document.createElement("div");
    row.className = "table__row";
    row.innerHTML = '<span style="grid-column:1 / -1">Sem encargos registados.</span>';
    chargesContainer.appendChild(row);
  }

  const taxesContainer = selectors("taxes");
  taxesContainer.innerHTML = "";
  (invoice.taxes_and_fees ?? []).forEach((item) => {
    const base = hasNumber(item.base_eur) ? currency(item.base_eur) : item.base_eur ?? "—";
    const vatAmount = hasNumber(item.vat_amount_eur)
      ? currency(item.vat_amount_eur)
      : item.vat_amount_eur ?? "—";
    const vatRate = hasNumber(item.vat_rate)
      ? `${number(item.vat_rate * 100, { maximumFractionDigits: 0 })}%`
      : item.vat_rate ?? "";
    const total = hasNumber(item.total_eur)
      ? currency(item.total_eur)
      : item.total_eur ?? "—";

    const row = document.createElement("div");
    row.className = "table__row";
    row.innerHTML = `
      <span>${item.description}</span>
      <span class="numeric">${base}</span>
      <span class="numeric">${vatAmount}<br/><small>${vatRate}</small></span>
      <span class="numeric">${total}</span>
    `;
    taxesContainer.appendChild(row);
  });
  if (!taxesContainer.children.length) {
    const row = document.createElement("div");
    row.className = "table__row";
    row.innerHTML = '<span style="grid-column:1 / -1">Sem taxas ou impostos registados.</span>';
    taxesContainer.appendChild(row);
  }

  selectors("paymentAmount").textContent = currency(invoice.payment?.amount_eur);
  selectors("paymentDeadline").textContent = formatDate(invoice.payment?.deadline);
  selectors("paymentChannels").textContent = joinList(invoice.payment?.channels);
  selectors("efficiencyTip").textContent = invoice.messages?.efficiency_tip ?? "—";
  selectors("securityNotice").textContent = invoice.messages?.security_notice ?? "—";

  const energyList = selectors("energyMix");
  energyList.innerHTML = "";
  (invoice.energy_mix_percentages ?? []).forEach((item) => {
    const li = document.createElement("li");
    li.innerHTML = `<strong>${number(item.percent, { maximumFractionDigits: 2 })}%</strong>${item.source}`;
    energyList.appendChild(li);
  });
  if (!energyList.children.length) {
    const li = document.createElement("li");
    li.textContent = "Mix energético não informado.";
    energyList.appendChild(li);
  }
  selectors("energyReference").textContent = invoice.energy_mix_reference ?? "—";

  selectors("atcudFull").textContent = invoice.atcud ?? "—";
  selectors("audiovisual").textContent = invoice.audiovisual_contribution
    ? `${invoice.audiovisual_contribution.invoice_number} — ${currency(
        invoice.audiovisual_contribution.total_eur,
      )}`
    : "—";
  selectors("additionalNotes").textContent = joinList(invoice.additional_notes);
}

async function bootstrap() {
  const statusPill = selectors("atcud");
  try {
    const response = await fetch("data/invoice-edp-2025-11.json", { cache: "no-cache" });
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    const invoice = await response.json();
    renderInvoice(invoice);
  } catch (error) {
    console.error("Falha ao carregar dados da fatura", error);
    if (statusPill) {
      statusPill.textContent = "Carga falhou";
      statusPill.classList.add("pill--error");
    }
    const main = document.querySelector("main");
    if (main) {
      const banner = document.createElement("div");
      banner.className = "panel";
      banner.innerHTML = `
        <header class="panel__header">
          <h2>Erro ao carregar dados</h2>
          <p>Não foi possível sincronizar o JSON da fatura a partir do GitHub Pages.</p>
        </header>
      `;
      main.prepend(banner);
    }
  }
}

document.addEventListener("DOMContentLoaded", bootstrap);
