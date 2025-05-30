import { useState } from "react";
import { useAppSettings } from "../../contextApi/appContext";
import VisitOverlay from "./visitOverlay/VisitOverlay";
import { TAppointmentWrapper } from "../../types";
import { History, Calendar } from "lucide-react";

type Tprops = {
  appointmentWarappers: TAppointmentWrapper[];
  isEdit: boolean;
};
function PatientVisitHistory({ appointmentWarappers, isEdit }: Tprops) {
  const { darkMode } = useAppSettings();
  const [isVisitOpen, setIsVisitOpen] = useState(false);
  //const [visitDate, setVisitDate] = useState("");
  // const [followupIds, setFollowupIds] = useState<string[]>([]);

  //const [appointmentId, setsetAppointmentId] = useState("");
  const [appointmentWarapperId, setAppointmentWarapperId] = useState("");

  const onOpenAppointmentHandler = (id: string, date: string) => {
    setIsVisitOpen(true);
    // setsetAppointmentId(mainId);
    // setVisitDate(date);
    //setFollowupIds(followupIds);
    setAppointmentWarapperId(id);
  };
  const onCloseVisitHandler = () => {
    setIsVisitOpen(!isVisitOpen);
  };
  return (
    <div className={`${darkMode ? "bg-gray-800" : "bg-white"} w-full `}>
      {isVisitOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50  z-50 ">
          <VisitOverlay
            // appointment_id={appointmentId}
            appointment_wrapper_id={appointmentWarapperId}
            onClose={onCloseVisitHandler}
          />
        </div>
      )}
      <h3 className="text-lg font-semibold mb-4 flex items-center text-gray-500 ">
        <History className="mr-2 text-green-700" size={18} />
        Appointments History
      </h3>

      {appointmentWarappers.length > 0 ? (
        <div
          className={`${isEdit ? "max-h-[380px]" : "max-h-[450px]"} space-y-6  overflow-y-auto custom-scrollbar pr-2`}
        >
          {appointmentWarappers.map((appoint, index) => (
            <div
              key={index}
              className={`${darkMode ? "border-gray-700" : "border-gray-200"} border-l-4 pl-4 ${
                index !== appointmentWarappers.length - 1 ? "pb-6" : ""
              }`}
            >
              <div>
                <div className="flex items-center mb-2 gap-4">
                  <Calendar
                    className={`mr-2 ${darkMode ? "text-green-700" : "text-green-300"}`}
                    size={16}
                  />
                  <p
                    className={`${darkMode ? "text-green-300" : "text-green-700"} cursor-pointer font-medium hover:text-green-400`}
                    onClick={() => {
                      onOpenAppointmentHandler(appoint.id, appoint.date);
                    }}
                  >
                    {appoint.date}
                  </p>
                </div>
                <p>
                  <span className="text-sm text-gray-400 mr-2 pb-8">
                    Case Status:{" "}
                  </span>
                  <span
                    className={`${appoint.appointment_status === "Closed" ? "text-red-500" : "text-green-500"} text-sm`}
                  >
                    {appoint.appointment_status}
                  </span>
                </p>
              </div>
              <div className=" gap-4">
                <div>
                  <p
                    className={` ${darkMode ? "text-gray-400" : "text-gray-700"} pb-1`}
                  >
                    Complaint:
                  </p>
                  <p
                    className={`${darkMode ? "text-gray-500" : "text-gray-500"} text-sm`}
                  >
                    {appoint.main_complaint}
                  </p>
                </div>
                <div></div>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <p className={`${darkMode ? "text-gray-400" : "text-gray-600"}`}>
          No visit history available
        </p>
      )}
    </div>
  );
}

export default PatientVisitHistory;
