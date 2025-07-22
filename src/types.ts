export interface StateRate {
  id: number;
  state: string;
  rate: number;
  is_variable: boolean;
  plus_percentage: number;
  update_frequency: string;
  last_update: string;
  notes: string;
}

export interface CalcRequest {
  judgment_date: string;
  is_federal: boolean;
  state: string;
  amount: number;
  from_date: string;
  to_date: string;
}

export interface CalcResponse {
  rate: number;
  days: number;
  interest_amount: number;
  total_amount: number;
  rate_source: string;
  disclaimer: string;
}

export interface ApiSettings {
  fred_api_key: string;
}
