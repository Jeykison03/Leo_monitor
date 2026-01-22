const int PULSE_PIN = A0;
const int LED_PIN = 13;

void setup() {
  pinMode(LED_PIN, OUTPUT);
  Serial.begin(9600);
}

void loop() {
  int signal = analogRead(PULSE_PIN);
  
  if (signal > 550) {
    digitalWrite(LED_PIN, HIGH);
  } else {
    digitalWrite(LED_PIN, LOW);
  }

  Serial.print("RAW:");
  Serial.println(signal);

  delay(20); 
}
