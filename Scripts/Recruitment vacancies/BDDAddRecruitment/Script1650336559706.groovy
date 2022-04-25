import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://opensource-demo.orangehrmlive.com/')

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_LOGIN Panel_txtUsername'), 
    'Admin')

WebUI.setEncryptedText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_Username_txtPassword'), 
    'hUKwJTbofgPU9eVlw/CnDQ==')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_Password_Submit'))

WebUI.click(findTestObject('HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/Page_OrangeHRM/b_Recruitment'))

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/a_Vacancies'))

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_Status_btnAdd'))

WebUI.selectOptionByValue(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/select_-- Select --Chief Executive OfficerC_a41235'), 
    '9', true)

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input__addJobVacancyname'), 
    'Senior QA Engineer')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/li_Hiring Manager'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input__addJobVacancyhiringManager'), 
    'Orange Test')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/li_Gavin Maury Dawkins'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_Number of Positions_addJobVacancynoOf_9b2c22'), 
    '1')

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/textarea_Description_addJobVacancydescription'), 
    'For contract staff')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Recruitment-Vacancies/Page_OrangeHRM/input_httpsopensource-demo.orangehrmlive.co_2ebf7b'))

WebUI.click(findTestObject('HRM ORANGE/Performance/Page_OrangeHRM/Page_OrangeHRM/div_Successfully Saved       Close'))

