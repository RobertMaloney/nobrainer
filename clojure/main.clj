(def initial-state
  {:tape (apply vector-of :byte (repeat 30000 0))
   :data-pointer 0})

(defn runcommand [program state pc]
  (let [dp (:data-pointer state)
        command (get program pc)]
    [(case command
      \+ (update-in state [:tape dp] inc)
      \- (update-in state [:tape dp] dec)
      \> (update-in state [:data-pointer] inc)
      \< (update-in state [:data-pointer] dec)
      \. (do
            (print (char (nth (:tape state) dp)))
            state)
      state)
    (case command
      pc)]))

(defn interpret [filename]
    (loop [in (slurp filename)
           state initial-state
           pc 0]
      (if (< pc (count in))
        (let [[newstate newpc] (runcommand in state pc)]
          (recur in newstate (+ newpc 1))))))
