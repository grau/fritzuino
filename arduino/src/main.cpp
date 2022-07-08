#include <Arduino.h>

int num = 0;
unsigned long previousMillis = 0;
unsigned long interval = 2000;

void setup() {
  pinMode(13, OUTPUT);
  pinMode(11, INPUT);
  digitalWrite(11, LOW);
  Serial.begin(115200);
}

void loop() {
  unsigned long currentMillis = millis();
  if (currentMillis - previousMillis > interval) {
    if (digitalRead(11) == LOW) {
      Serial.println(num++);
    }
    previousMillis = currentMillis;
  }

  if (digitalRead(11) == HIGH) {
    digitalWrite(13, HIGH);
    Serial.println("RING");
    delay(2000);
  }
}