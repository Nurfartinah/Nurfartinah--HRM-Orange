<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Search Performance Reviews             _3be71a</name>
   <tag></tag>
   <elementGuidId>a2286ab7-bb13-4ca0-9f1d-823476ceaf31</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#content</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='content']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>f35710f5-5b88-4413-bff9-19332777ce29</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>content</value>
      <webElementGuid>f11059eb-9f71-4dba-8efd-69df42bf5c70</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

                  
 


    
        Search Performance Reviews
    
    
        
                            
                
                    Employee Name
  

Job Title
  
All
Automation Tester
BTest
Chief Executive Officer
Chief Financial Officer
Content Specialist
Customer Success Manager
Database Administrator
EA
Engineer
Finance Manager
Financial Analyst
Head of Support
HR Associate
HR Manager
IT Manager
Network Administrator
Payroll Administrator
Pre-Sales Coordinator
QA Engineer
QA Lead
Sales Representative
Social Media Marketer
Software Architect
Software Engineer
Support Specialist
VP - Client Services
VP - Sales &amp; Marketing


Department
  
All
Administration
Engineering
  Development
  Quality Assurance
  TechOps
Sales &amp; Marketing
  Sales
  Marketing
Client Services
  Technical Support
Finance
Human Resources


Status
  
All
Activated
Approved
In Progress


From Date
   

    var datepickerDateFormat = 'yy-mm-dd';
    var displayDateFormat = datepickerDateFormat.replace('yy', 'yyyy');

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#fromDate&quot;).val());
        if (dateFieldValue == '') {
            $(&quot;#fromDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#fromDate&quot;,
        {
            showOn: &quot;both&quot;,
            dateFormat: datepickerDateFormat,
            buttonImage: &quot;/webres_625505b4d6d5f1.18812754/themes/default/images/calendar.png&quot;,
            buttonText:&quot;&quot;,
            buttonImageOnly: true,
            changeMonth: true,
            changeYear: true,
            yearRange: &quot;-100:+100&quot;,
            firstDay: 1,
            onClose: function() {
                $(&quot;#fromDate&quot;).trigger('blur');
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#fromDate&quot;).click(function(){
            daymarker.show(&quot;#fromDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val('');
            }
        });
    
    });



To Date
   

    var datepickerDateFormat = 'yy-mm-dd';
    var displayDateFormat = datepickerDateFormat.replace('yy', 'yyyy');

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#toDate&quot;).val());
        if (dateFieldValue == '') {
            $(&quot;#toDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#toDate&quot;,
        {
            showOn: &quot;both&quot;,
            dateFormat: datepickerDateFormat,
            buttonImage: &quot;/webres_625505b4d6d5f1.18812754/themes/default/images/calendar.png&quot;,
            buttonText:&quot;&quot;,
            buttonImageOnly: true,
            changeMonth: true,
            changeYear: true,
            yearRange: &quot;-100:+100&quot;,
            firstDay: 1,
            onClose: function() {
                $(&quot;#toDate&quot;).trigger('blur');
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#toDate&quot;).click(function(){
            daymarker.show(&quot;#toDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val('');
            }
        });
    
    });




                    
                         
                                            
                
                     
                    
                                
            

        

     
    >



   
    
    
            Review List
        
    
    
        
        


                
            
    
                 

        


         
        

        
            
            
                
        
        
            
                                
                Employee
Due Date
Review Period
Job Title
Department
Status
Evaluation Status
            
                            

            
                                        
                            No Records Found
                        
                                
                            
         
                  
                
        
     
        
 




                    

                        var rootPath = '/';

                        $(document).ready(function() {
                            ohrmList_init();

                        });
                                              




    $(document).ready(function() {
        
          
        var employees = [] ;        

        if ($(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).val() == '') {
            $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).val('Type for hints...').addClass(&quot;inputFormatHint&quot;);
        }
        
        $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).autocomplete(employees, {
            formatItem: function(item) {
                return item.name;
            }
            ,matchContains:true
        }).result(function(event, item) {
        }
    );
       
                            
        $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).one('focus', function() {
            if ($(this).hasClass(&quot;inputFormatHint&quot;)) {
                $(this).val(&quot;&quot;);
                $(this).removeClass(&quot;inputFormatHint&quot;);
            }
        }); 
        
        $('#searchBtn').click(function(){              
            $(&quot;#evaluatePerformanceReview360SearchForm_employeeName.inputFormatHint&quot;).val('');
            $('#evaluatePerformanceReview360SearchForm').submit();
        });   
        
        $('#addBtn').click(function(){
            $('#evaluatePerformanceReview360SearchForm').attr(&quot;action&quot;, &quot;/index.php/performance/saveReview&quot;);
            $('#evaluatePerformanceReview360SearchForm').attr(&quot;method&quot;, &quot;get&quot;);
            $('#evaluatePerformanceReview360SearchForm').submit();
        });
        
        $('#deleteBtn').click(function(){
            $('#frmList_ohrmListComponent').attr(&quot;action&quot;, &quot;/index.php/performance/deleteReview&quot;);
            $('#frmList_ohrmListComponent').submit();
        });
        
        $('#btnReset').click(function(){
        $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).val('');
        $(&quot;#evaluatePerformanceReview360SearchForm_jobTitleCode&quot;).val('');
        $(&quot;#evaluatePerformanceReview360SearchForm_reviwerName&quot;).val('');
        $(&quot;#evaluatePerformanceReview360SearchForm_department&quot;).val('0');
        $(&quot;#evaluatePerformanceReview360SearchForm_status&quot;).val('0');
        $(&quot;#fromDate&quot;).val('yyyy-mm-dd');
        $(&quot;#toDate&quot;).val('yyyy-mm-dd');
        $('#evaluatePerformanceReview360SearchForm').submit();
        });
    });
    
    function submitPage(pageNo) {
            document.evaluatePerformanceReviewSearchForm.pageNo.value = pageNo;
            document.evaluatePerformanceReviewSearchForm.hdnAction.value = 'paging';
            $('#evaluatePerformanceReview360SearchForm input.inputFormatHint').val('');
            $('#evaluatePerformanceReview360SearchForm input.ac_loading').val('');
            document.getElementById('evaluatePerformanceReview360SearchForm').submit();
        }

            </value>
      <webElementGuid>4722bb12-6037-44ce-97f7-b4083115737a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;content&quot;)</value>
      <webElementGuid>7d1303a0-bc37-4e1a-8bb8-daf593b1b549</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='content']</value>
      <webElementGuid>26282b2a-7824-4290-a1a7-5c0bb180eff2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='wrapper']/div[3]</value>
      <webElementGuid>f8fdb3c6-12f1-41f3-9b02-4e3c79abbed0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Buzz'])[1]/following::div[1]</value>
      <webElementGuid>c73eea67-0b53-4592-99d5-c31376480582</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Directory'])[1]/following::div[1]</value>
      <webElementGuid>2c957ac6-ea02-445e-969e-daa29ef2075c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[3]</value>
      <webElementGuid>dd68edb9-e6b2-47aa-9427-0d76fffdb2df</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'content' and (text() = concat(&quot;

                  
 


    
        Search Performance Reviews
    
    
        
                            
                
                    Employee Name
  

Job Title
  
All
Automation Tester
BTest
Chief Executive Officer
Chief Financial Officer
Content Specialist
Customer Success Manager
Database Administrator
EA
Engineer
Finance Manager
Financial Analyst
Head of Support
HR Associate
HR Manager
IT Manager
Network Administrator
Payroll Administrator
Pre-Sales Coordinator
QA Engineer
QA Lead
Sales Representative
Social Media Marketer
Software Architect
Software Engineer
Support Specialist
VP - Client Services
VP - Sales &amp; Marketing


Department
  
All
Administration
Engineering
  Development
  Quality Assurance
  TechOps
Sales &amp; Marketing
  Sales
  Marketing
Client Services
  Technical Support
Finance
Human Resources


Status
  
All
Activated
Approved
In Progress


From Date
   

    var datepickerDateFormat = &quot; , &quot;'&quot; , &quot;yy-mm-dd&quot; , &quot;'&quot; , &quot;;
    var displayDateFormat = datepickerDateFormat.replace(&quot; , &quot;'&quot; , &quot;yy&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;yyyy&quot; , &quot;'&quot; , &quot;);

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#fromDate&quot;).val());
        if (dateFieldValue == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#fromDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#fromDate&quot;,
        {
            showOn: &quot;both&quot;,
            dateFormat: datepickerDateFormat,
            buttonImage: &quot;/webres_625505b4d6d5f1.18812754/themes/default/images/calendar.png&quot;,
            buttonText:&quot;&quot;,
            buttonImageOnly: true,
            changeMonth: true,
            changeYear: true,
            yearRange: &quot;-100:+100&quot;,
            firstDay: 1,
            onClose: function() {
                $(&quot;#fromDate&quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#fromDate&quot;).click(function(){
            daymarker.show(&quot;#fromDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    
    });



To Date
   

    var datepickerDateFormat = &quot; , &quot;'&quot; , &quot;yy-mm-dd&quot; , &quot;'&quot; , &quot;;
    var displayDateFormat = datepickerDateFormat.replace(&quot; , &quot;'&quot; , &quot;yy&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;yyyy&quot; , &quot;'&quot; , &quot;);

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#toDate&quot;).val());
        if (dateFieldValue == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#toDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#toDate&quot;,
        {
            showOn: &quot;both&quot;,
            dateFormat: datepickerDateFormat,
            buttonImage: &quot;/webres_625505b4d6d5f1.18812754/themes/default/images/calendar.png&quot;,
            buttonText:&quot;&quot;,
            buttonImageOnly: true,
            changeMonth: true,
            changeYear: true,
            yearRange: &quot;-100:+100&quot;,
            firstDay: 1,
            onClose: function() {
                $(&quot;#toDate&quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#toDate&quot;).click(function(){
            daymarker.show(&quot;#toDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    
    });




                    
                         
                                            
                
                     
                    
                                
            

        

     
    >



   
    
    
            Review List
        
    
    
        
        


                
            
    
                 

        


         
        

        
            
            
                
        
        
            
                                
                Employee
Due Date
Review Period
Job Title
Department
Status
Evaluation Status
            
                            

            
                                        
                            No Records Found
                        
                                
                            
         
                  
                
        
     
        
 




                    

                        var rootPath = &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;;

                        $(document).ready(function() {
                            ohrmList_init();

                        });
                                              




    $(document).ready(function() {
        
          
        var employees = [] ;        

        if ($(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).val(&quot; , &quot;'&quot; , &quot;Type for hints...&quot; , &quot;'&quot; , &quot;).addClass(&quot;inputFormatHint&quot;);
        }
        
        $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).autocomplete(employees, {
            formatItem: function(item) {
                return item.name;
            }
            ,matchContains:true
        }).result(function(event, item) {
        }
    );
       
                            
        $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).one(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {
            if ($(this).hasClass(&quot;inputFormatHint&quot;)) {
                $(this).val(&quot;&quot;);
                $(this).removeClass(&quot;inputFormatHint&quot;);
            }
        }); 
        
        $(&quot; , &quot;'&quot; , &quot;#searchBtn&quot; , &quot;'&quot; , &quot;).click(function(){              
            $(&quot;#evaluatePerformanceReview360SearchForm_employeeName.inputFormatHint&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).submit();
        });   
        
        $(&quot; , &quot;'&quot; , &quot;#addBtn&quot; , &quot;'&quot; , &quot;).click(function(){
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).attr(&quot;action&quot;, &quot;/index.php/performance/saveReview&quot;);
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).attr(&quot;method&quot;, &quot;get&quot;);
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).submit();
        });
        
        $(&quot; , &quot;'&quot; , &quot;#deleteBtn&quot; , &quot;'&quot; , &quot;).click(function(){
            $(&quot; , &quot;'&quot; , &quot;#frmList_ohrmListComponent&quot; , &quot;'&quot; , &quot;).attr(&quot;action&quot;, &quot;/index.php/performance/deleteReview&quot;);
            $(&quot; , &quot;'&quot; , &quot;#frmList_ohrmListComponent&quot; , &quot;'&quot; , &quot;).submit();
        });
        
        $(&quot; , &quot;'&quot; , &quot;#btnReset&quot; , &quot;'&quot; , &quot;).click(function(){
        $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot;#evaluatePerformanceReview360SearchForm_jobTitleCode&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot;#evaluatePerformanceReview360SearchForm_reviwerName&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot;#evaluatePerformanceReview360SearchForm_department&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
        $(&quot;#evaluatePerformanceReview360SearchForm_status&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
        $(&quot;#fromDate&quot;).val(&quot; , &quot;'&quot; , &quot;yyyy-mm-dd&quot; , &quot;'&quot; , &quot;);
        $(&quot;#toDate&quot;).val(&quot; , &quot;'&quot; , &quot;yyyy-mm-dd&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).submit();
        });
    });
    
    function submitPage(pageNo) {
            document.evaluatePerformanceReviewSearchForm.pageNo.value = pageNo;
            document.evaluatePerformanceReviewSearchForm.hdnAction.value = &quot; , &quot;'&quot; , &quot;paging&quot; , &quot;'&quot; , &quot;;
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm input.inputFormatHint&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm input.ac_loading&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            document.getElementById(&quot; , &quot;'&quot; , &quot;evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).submit();
        }

            &quot;) or . = concat(&quot;

                  
 


    
        Search Performance Reviews
    
    
        
                            
                
                    Employee Name
  

Job Title
  
All
Automation Tester
BTest
Chief Executive Officer
Chief Financial Officer
Content Specialist
Customer Success Manager
Database Administrator
EA
Engineer
Finance Manager
Financial Analyst
Head of Support
HR Associate
HR Manager
IT Manager
Network Administrator
Payroll Administrator
Pre-Sales Coordinator
QA Engineer
QA Lead
Sales Representative
Social Media Marketer
Software Architect
Software Engineer
Support Specialist
VP - Client Services
VP - Sales &amp; Marketing


Department
  
All
Administration
Engineering
  Development
  Quality Assurance
  TechOps
Sales &amp; Marketing
  Sales
  Marketing
Client Services
  Technical Support
Finance
Human Resources


Status
  
All
Activated
Approved
In Progress


From Date
   

    var datepickerDateFormat = &quot; , &quot;'&quot; , &quot;yy-mm-dd&quot; , &quot;'&quot; , &quot;;
    var displayDateFormat = datepickerDateFormat.replace(&quot; , &quot;'&quot; , &quot;yy&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;yyyy&quot; , &quot;'&quot; , &quot;);

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#fromDate&quot;).val());
        if (dateFieldValue == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#fromDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#fromDate&quot;,
        {
            showOn: &quot;both&quot;,
            dateFormat: datepickerDateFormat,
            buttonImage: &quot;/webres_625505b4d6d5f1.18812754/themes/default/images/calendar.png&quot;,
            buttonText:&quot;&quot;,
            buttonImageOnly: true,
            changeMonth: true,
            changeYear: true,
            yearRange: &quot;-100:+100&quot;,
            firstDay: 1,
            onClose: function() {
                $(&quot;#fromDate&quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#fromDate&quot;).click(function(){
            daymarker.show(&quot;#fromDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    
    });



To Date
   

    var datepickerDateFormat = &quot; , &quot;'&quot; , &quot;yy-mm-dd&quot; , &quot;'&quot; , &quot;;
    var displayDateFormat = datepickerDateFormat.replace(&quot; , &quot;'&quot; , &quot;yy&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;yyyy&quot; , &quot;'&quot; , &quot;);

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#toDate&quot;).val());
        if (dateFieldValue == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#toDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#toDate&quot;,
        {
            showOn: &quot;both&quot;,
            dateFormat: datepickerDateFormat,
            buttonImage: &quot;/webres_625505b4d6d5f1.18812754/themes/default/images/calendar.png&quot;,
            buttonText:&quot;&quot;,
            buttonImageOnly: true,
            changeMonth: true,
            changeYear: true,
            yearRange: &quot;-100:+100&quot;,
            firstDay: 1,
            onClose: function() {
                $(&quot;#toDate&quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#toDate&quot;).click(function(){
            daymarker.show(&quot;#toDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    
    });




                    
                         
                                            
                
                     
                    
                                
            

        

     
    >



   
    
    
            Review List
        
    
    
        
        


                
            
    
                 

        


         
        

        
            
            
                
        
        
            
                                
                Employee
Due Date
Review Period
Job Title
Department
Status
Evaluation Status
            
                            

            
                                        
                            No Records Found
                        
                                
                            
         
                  
                
        
     
        
 




                    

                        var rootPath = &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;;

                        $(document).ready(function() {
                            ohrmList_init();

                        });
                                              




    $(document).ready(function() {
        
          
        var employees = [] ;        

        if ($(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).val(&quot; , &quot;'&quot; , &quot;Type for hints...&quot; , &quot;'&quot; , &quot;).addClass(&quot;inputFormatHint&quot;);
        }
        
        $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).autocomplete(employees, {
            formatItem: function(item) {
                return item.name;
            }
            ,matchContains:true
        }).result(function(event, item) {
        }
    );
       
                            
        $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).one(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {
            if ($(this).hasClass(&quot;inputFormatHint&quot;)) {
                $(this).val(&quot;&quot;);
                $(this).removeClass(&quot;inputFormatHint&quot;);
            }
        }); 
        
        $(&quot; , &quot;'&quot; , &quot;#searchBtn&quot; , &quot;'&quot; , &quot;).click(function(){              
            $(&quot;#evaluatePerformanceReview360SearchForm_employeeName.inputFormatHint&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).submit();
        });   
        
        $(&quot; , &quot;'&quot; , &quot;#addBtn&quot; , &quot;'&quot; , &quot;).click(function(){
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).attr(&quot;action&quot;, &quot;/index.php/performance/saveReview&quot;);
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).attr(&quot;method&quot;, &quot;get&quot;);
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).submit();
        });
        
        $(&quot; , &quot;'&quot; , &quot;#deleteBtn&quot; , &quot;'&quot; , &quot;).click(function(){
            $(&quot; , &quot;'&quot; , &quot;#frmList_ohrmListComponent&quot; , &quot;'&quot; , &quot;).attr(&quot;action&quot;, &quot;/index.php/performance/deleteReview&quot;);
            $(&quot; , &quot;'&quot; , &quot;#frmList_ohrmListComponent&quot; , &quot;'&quot; , &quot;).submit();
        });
        
        $(&quot; , &quot;'&quot; , &quot;#btnReset&quot; , &quot;'&quot; , &quot;).click(function(){
        $(&quot;#evaluatePerformanceReview360SearchForm_employeeName&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot;#evaluatePerformanceReview360SearchForm_jobTitleCode&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot;#evaluatePerformanceReview360SearchForm_reviwerName&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot;#evaluatePerformanceReview360SearchForm_department&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
        $(&quot;#evaluatePerformanceReview360SearchForm_status&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
        $(&quot;#fromDate&quot;).val(&quot; , &quot;'&quot; , &quot;yyyy-mm-dd&quot; , &quot;'&quot; , &quot;);
        $(&quot;#toDate&quot;).val(&quot; , &quot;'&quot; , &quot;yyyy-mm-dd&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).submit();
        });
    });
    
    function submitPage(pageNo) {
            document.evaluatePerformanceReviewSearchForm.pageNo.value = pageNo;
            document.evaluatePerformanceReviewSearchForm.hdnAction.value = &quot; , &quot;'&quot; , &quot;paging&quot; , &quot;'&quot; , &quot;;
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm input.inputFormatHint&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#evaluatePerformanceReview360SearchForm input.ac_loading&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            document.getElementById(&quot; , &quot;'&quot; , &quot;evaluatePerformanceReview360SearchForm&quot; , &quot;'&quot; , &quot;).submit();
        }

            &quot;))]</value>
      <webElementGuid>01296d17-cd76-4819-a803-3d32b72843c7</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
