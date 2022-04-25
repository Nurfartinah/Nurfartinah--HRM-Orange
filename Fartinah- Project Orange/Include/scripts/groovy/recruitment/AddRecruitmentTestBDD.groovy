package recruitment
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When



class AddRecruitmentTestBDD {
	/**
	 * The step definitions below match with Katalon sample Gherkin steps
	 */

	@Given("I want to navigate to Vacancies page")
	def navigateToLoginPage(){
		WebUI.openBrowser('')
		//WebUI.navigateToUrl("https://opensource-demo.orangehrmlive.com/")
		WebUI.callTestCase(findTestCase('Common/Login HRM web'), [:], FailureHandling.STOP_ON_FAILURE)
		WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/Page_OrangeHRM/b_Recruitment'))
		WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/a_Vacancies'))
		WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_Status_btnAdd'))
	}



	@When("User select job title")
	//@When("User select job title and click (.*) and (.*) and (.*) and (.*)")
	//def enterVacancies(String vacancyname, String hiringmanager, String noposition, String description){
	def clickInfo(){
		//WebUI.selectOptionByValue(findTestObject('Page_CuraAppointment/lst_Facility'), 'Hongkong CURA Healthcare Center', false)
		//WebUI.selectOptionByLabel(findTestObject('dropdown_Month'), 'Apr', false)
		//WebUI.selectOptionByLabel(findTestObject('addJobVacancy_jobTitle'), 'Chief Executive Officer', false)
		//WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/select_-- Select --Chief Executive OfficerC_a41235'), jobtitle)
		//WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input__addJobVacancyname'), vacancyname)
		//WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/li_Hiring Manager'), hiringmanager)
		//WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_Number of Positions_addJobVacancynoOf_9b2c22'), noposition)
		//WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/textarea_Description_addJobVacancydescription'), description)

		WebUI.selectOptionByValue(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/select_-- Select --Chief Executive OfficerC_a41235'),
				'9', true)
		WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input__addJobVacancyname'),
				'Senior QA Engineer')
		WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/li_Hiring Manager'))
		WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input__addJobVacancyhiringManager'),
				'Orange Test')
		WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_Number of Positions_addJobVacancynoOf_9b2c22'),
				'1')
		WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/textarea_Description_addJobVacancydescription'),
				'For contract staff')

	}

	@And("Click on save button")
	def ClickSave() {
		WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_httpsopensource-demo.orangehrmlive.co_2ebf7b'))
	}

	@Then("User navigate to edit job vacancy")
	def verifyHomePage() {
		WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_httpsopensource-demo.orangehrmlive.co_2ebf7b'))
		WebUI.click(findTestObject('HRM ORANGE/Performance/Page_OrangeHRM/Page_OrangeHRM/div_Successfully Saved       Close'))
		WebUI.closeBrowser()
	}

	@Then("Warning message is displayed")
	def verifyFailPage() {
		WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/Page_OrangeHRM/span_Already exists'))
		WebUI.closeBrowser()
	}
}