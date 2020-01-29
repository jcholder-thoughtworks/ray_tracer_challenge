Feature: Colors

  Scenario: Adding colors
    Given c1 = color(0.9, 0.6, 0.75)
    And c2 = color(0.7, 0.1, 0.25)
    Then c1 + c2 = color(1.6, 0.7, 1.0)
