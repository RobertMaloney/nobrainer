(def initial-state
  {:tape (apply vector-of :byte (repeat 30000 0))
   :data-pointer 0})

(defn runcommand [command state]
  (let [dp (:data-pointer state)]
    (case command
      \+ (update-in state [:tape dp] inc)
      \- (update-in state [:tape dp] dec)
      \> (update-in state [:data-pointer] inc)
      \< (update-in state [:data-pointer] dec)
      \. (do
            (print (char (nth (:tape state) dp)))
            state)
      state)))

(defn interpret [filename]
  (loop [in (slurp filename)
         state initial-state]
    (if (> (count in) 0)
      (recur (rest in) (runcommand (first in) state)))))
