use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Patient {
    pub id: String,
    pub name: String,
    pub dob: String,
    pub age: String,
    pub gender: String,
    pub marital_status: String,
    pub born_city: String,
    pub residence: String,
    pub occupation: String,
    pub phone: String,
    pub email: String,
    pub insurance_provider: String,
    pub insurance_policy_number: String,
    pub insurance_group_number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Med {
    pub med_name: String,
    pub dosage: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PatientMedicalHistory {
    pub id: String,
    pub patient_id: String,
    pub allergies: Option<Vec<String>>,
    pub medications: Option<Vec<Med>>,
    pub conditions: Option<Vec<String>>,
    pub special_habits: Option<Vec<String>>,
    pub family_history: Option<String>,
    pub notes: Option<String>,
}

// ---------------------Appointment
#[derive(Debug, Deserialize, Serialize)]
pub struct AppointmentWrapper {
    pub id: String,
    pub patient_id: String,
    pub main_complaint: String,
    pub main_appointment: String,
    pub followups_num: String,
    pub followup_appointments: Vec<String>,
    pub appointment_status: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Appointment {
    pub id: String,
    pub patient_id: String,
    pub vitals: Vec<Vitals>,
    pub complaint: Option<String>,
    pub present_history: Option<String>,
    pub examination: Option<String>,
    pub provisional_diagnosis: Option<String>,
    pub prescription: Option<Vec<Prescription>>,
    pub requests: Option<String>,
    pub services: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vitals {
    pub v_name: String,
    pub v_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Diagnoses {
    pub id: String,
    pub patient_id: String,
    pub diagnoses: Option<Vec<Diagnosis>>,
    pub date: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Diagnosis {
    pub id: Option<String>,
    pub diagnosis_type: Option<String>,
    pub diagnosis_title: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub ongoing: bool,
    pub comment: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AllRequests {
    pub id: String,
    pub requests: Option<Vec<Request>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: Option<String>,
    pub req_date: Option<String>,
    pub req_name: Option<String>,
    pub comment: Option<String>,
    pub req_type: Option<String>,
    pub resualt: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Prescription {
    pub name: String,
    pub dosage: String,
    pub duration: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppointmentDay {
    pub id: String,
    pub day: String,
    pub patient_data: Vec<PatientData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatientData {
    pub patient_id: String,
    pub name: String,
    pub appointment_type: String,
    pub description: String,
    pub time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppointmentsTime {
    pub from: Option<String>,
    pub to: Option<String>,
    pub excepting: Option<Vec<String>>,
}

//-----------Clinic Mangement
#[derive(Debug, Deserialize, Serialize)]
pub struct ClinicInfo {
    pub id: String,
    pub clinic_name: Option<String>,
    pub speciality: Option<String>,
    pub memberships: Option<Vec<String>>,
    pub address: Option<String>,
    pub contactus: Option<Vec<String>>,
    pub appointments: AppointmentsTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Employee {
    pub id: String,
    pub name: Option<String>,
    pub n_id: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub role: Option<String>,
    pub salary: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeeAndServices {
    pub id: String,
    pub fee: String,
    pub followups: Vec<Folloup>,
    pub services: Vec<Service>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppointmentFees {
    pub id: String,
    pub patient_id: String,
    pub patient_name: String,
    pub patient_phone: String,
    pub appointment_type: String,
    pub fee: String,
    pub services: Vec<Service>,
    pub total_fees: String,
    pub date: String,
    pub time_stamp: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Folloup {
    pub followup_name: String,
    pub followup_fee: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub service_name: String,
    pub service_fee: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Purchases {
    pub item_name: String,
    pub price: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Installment {
    pub insinternet_name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OtherExpense {
    pub other_expense_name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Expenses {
    pub id: String,
    pub month: Option<String>,
    pub rent: Option<String>,
    pub taxes: Option<String>,
    pub electricity_invoice: Option<String>,
    pub water_invoice: Option<String>,
    pub phone_and_internet_invoice: Option<String>,
    pub purchases: Option<Vec<Purchases>>,
    pub installments: Option<Vec<Installment>>,
    pub other_expenses: Option<Vec<OtherExpense>>,
}
