Feature: Recruitment vacancies

  @tag1
  Scenario Outline: Add New Vacancy
    Given I want to navigate to Vacancies page
    When User select job title
    And Click on save button
    Then User navigate to edit job vacancy

    Examples: 
      | vacancyname        | hiringmanager | noposition | description        |
      | Senior QA Engineer | Orange Test   |          1 | For contract staff |

  Scenario Outline: Add New Vacancy with existing data
    Given I want to navigate to Vacancies page
    When User select job title
    And Click on save button
    Then Warning message is displayed

    Examples: 
      | vacancyname        | hiringmanager | noposition | description |
      | Senior QA Engineer | Orange Test   |            |             |
