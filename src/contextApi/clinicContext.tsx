import { createContext, ReactNode, useState, useMemo, useContext } from "react";
import { TPatientInfo } from "../types";
import { patientInit } from "../initData";
//import { TPatientInfo } from "../validations/patientInfoSchema";

interface ClinicContextType {
  patientInfo: TPatientInfo | undefined;
  setPatientInfo: (patient: TPatientInfo) => void;
}

const ClinicContext = createContext<ClinicContextType | undefined>(undefined);

function ClinicProvider({ children }: { children: ReactNode }) {
  const [patientInfo, setPatientInfo] = useState<TPatientInfo | undefined>(
    patientInit,
  );

  const memoizedValue = useMemo(
    () => ({
      patientInfo,
      setPatientInfo,
    }),
    [patientInfo, setPatientInfo],
  );

  return (
    <ClinicContext.Provider value={memoizedValue}>
      {children}
    </ClinicContext.Provider>
  );
}

const useClinic = () => {
  const context = useContext(ClinicContext);
  if (!context) {
    throw new Error("useCounter must be used within a CounterProvider");
  }
  return context;
};

export { ClinicProvider, useClinic };
