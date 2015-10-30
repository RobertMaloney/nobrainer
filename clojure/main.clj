(def initial-state
  {:tape (apply vector-of :byte (repeat 30000 0))
   :data-pointer 0})

(defn goback [program state pc]
  (if (not= 0 (get (:tape state) (:data-pointer state)))
    (loop [counter 1
           pointer (dec pc)]
      (if (> counter 0)
        (recur (case (get program pointer)
          \[ (dec counter)
          \] (inc counter)
          counter) (dec pointer))
        pointer))
    pc))

(defn goforward [program state pc]
  (if (= 0 (get (:tape state) (:data-pointer state)))
    (loop [counter 1
           pointer (inc pc)]
      (if (> counter 0)
        (recur (case (get program pointer)
          \[ (inc counter)
          \] (dec counter)
          counter) (inc pointer))
        pointer))
    pc))

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
      \[ (goforward program state pc)
      \] (goback program state pc)
      pc)]))

(defn interpret [filename]
    (loop [in (slurp filename)
           state initial-state
           pc 0]
      (if (< pc (count in))
        (let [[newstate newpc] (runcommand in state pc)]
          (recur in newstate (inc newpc))))))
