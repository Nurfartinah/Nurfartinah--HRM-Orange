<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>ol_From       var datepickerDateFormat  yy-_ab6524</name>
   <tag></tag>
   <elementGuidId>9533084c-afb5-4871-b6fb-268b66c3056f</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#frmFilterLeave > fieldset > ol</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//form[@id='frmFilterLeave']/fieldset/ol</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>ol</value>
      <webElementGuid>c8095623-dff1-443b-8da8-c91493464c19</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                    From
   

    var datepickerDateFormat = 'yy-mm-dd';
    var displayDateFormat = datepickerDateFormat.replace('yy', 'yyyy');

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#calFromDate&quot;).val());
        if (dateFieldValue == '') {
            $(&quot;#calFromDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#calFromDate&quot;,
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
                $(&quot;#calFromDate&quot;).trigger('blur');
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#calFromDate&quot;).click(function(){
            daymarker.show(&quot;#calFromDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val('');
            }
        });
    
    });



To
   

    var datepickerDateFormat = 'yy-mm-dd';
    var displayDateFormat = datepickerDateFormat.replace('yy', 'yyyy');

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#calToDate&quot;).val());
        if (dateFieldValue == '') {
            $(&quot;#calToDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#calToDate&quot;,
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
                $(&quot;#calToDate&quot;).trigger('blur');
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#calToDate&quot;).click(function(){
            daymarker.show(&quot;#calToDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val('');
            }
        });
    
    });



Show Leave with Status
  All 
Rejected 
Cancelled 
Pending Approval 
Scheduled 
Taken 

$(document).ready(function() {

    $('#leaveList_chkSearchFilter_checkboxgroup_allcheck').click(function() {
       $('#leaveList_chkSearchFilter_checkboxgroup input[type=&quot;checkbox&quot;]').prop('checked', $(this).is(&quot;:checked&quot;));
    });
                                
    $('#leaveList_chkSearchFilter_checkboxgroup input[type=&quot;checkbox&quot;]').click(function() {
        var notCheckedCount = $('#leaveList_chkSearchFilter_checkboxgroup input[name=&quot;leaveList[chkSearchFilter][]&quot;]:not(:checked)').length;

        $('#leaveList_chkSearchFilter_checkboxgroup_allcheck').prop('checked', notCheckedCount === 0);
    });
});

 

                </value>
      <webElementGuid>d8ed5c5a-d03c-4a65-8794-9e74dc93bbe8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;frmFilterLeave&quot;)/fieldset[1]/ol[1]</value>
      <webElementGuid>ec34f589-a692-4498-b661-041b3d043cfd</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='frmFilterLeave']/fieldset/ol</value>
      <webElementGuid>f57265ec-8851-4af0-9d25-3bf0e982d9e3</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='My Leave List'])[1]/following::ol[1]</value>
      <webElementGuid>5b4ee6bb-c1fd-4929-8bd8-32ead9f67bdf</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Buzz'])[1]/following::ol[1]</value>
      <webElementGuid>c80d204c-59e5-46db-8eca-9a96d6821e97</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[3]/div/div[2]/form/fieldset/ol</value>
      <webElementGuid>227f87f0-0058-40d7-83f3-3b17b6374f65</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//ol[(text() = concat(&quot;
                    From
   

    var datepickerDateFormat = &quot; , &quot;'&quot; , &quot;yy-mm-dd&quot; , &quot;'&quot; , &quot;;
    var displayDateFormat = datepickerDateFormat.replace(&quot; , &quot;'&quot; , &quot;yy&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;yyyy&quot; , &quot;'&quot; , &quot;);

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#calFromDate&quot;).val());
        if (dateFieldValue == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#calFromDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#calFromDate&quot;,
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
                $(&quot;#calFromDate&quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#calFromDate&quot;).click(function(){
            daymarker.show(&quot;#calFromDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    
    });



To
   

    var datepickerDateFormat = &quot; , &quot;'&quot; , &quot;yy-mm-dd&quot; , &quot;'&quot; , &quot;;
    var displayDateFormat = datepickerDateFormat.replace(&quot; , &quot;'&quot; , &quot;yy&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;yyyy&quot; , &quot;'&quot; , &quot;);

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#calToDate&quot;).val());
        if (dateFieldValue == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#calToDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#calToDate&quot;,
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
                $(&quot;#calToDate&quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#calToDate&quot;).click(function(){
            daymarker.show(&quot;#calToDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    
    });



Show Leave with Status
  All 
Rejected 
Cancelled 
Pending Approval 
Scheduled 
Taken 

$(document).ready(function() {

    $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup_allcheck&quot; , &quot;'&quot; , &quot;).click(function() {
       $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup input[type=&quot;checkbox&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, $(this).is(&quot;:checked&quot;));
    });
                                
    $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup input[type=&quot;checkbox&quot;]&quot; , &quot;'&quot; , &quot;).click(function() {
        var notCheckedCount = $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup input[name=&quot;leaveList[chkSearchFilter][]&quot;]:not(:checked)&quot; , &quot;'&quot; , &quot;).length;

        $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup_allcheck&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, notCheckedCount === 0);
    });
});

 

                &quot;) or . = concat(&quot;
                    From
   

    var datepickerDateFormat = &quot; , &quot;'&quot; , &quot;yy-mm-dd&quot; , &quot;'&quot; , &quot;;
    var displayDateFormat = datepickerDateFormat.replace(&quot; , &quot;'&quot; , &quot;yy&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;yyyy&quot; , &quot;'&quot; , &quot;);

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#calFromDate&quot;).val());
        if (dateFieldValue == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#calFromDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#calFromDate&quot;,
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
                $(&quot;#calFromDate&quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#calFromDate&quot;).click(function(){
            daymarker.show(&quot;#calFromDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    
    });



To
   

    var datepickerDateFormat = &quot; , &quot;'&quot; , &quot;yy-mm-dd&quot; , &quot;'&quot; , &quot;;
    var displayDateFormat = datepickerDateFormat.replace(&quot; , &quot;'&quot; , &quot;yy&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;yyyy&quot; , &quot;'&quot; , &quot;);

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#calToDate&quot;).val());
        if (dateFieldValue == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#calToDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#calToDate&quot;,
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
                $(&quot;#calToDate&quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#calToDate&quot;).click(function(){
            daymarker.show(&quot;#calToDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    
    });



Show Leave with Status
  All 
Rejected 
Cancelled 
Pending Approval 
Scheduled 
Taken 

$(document).ready(function() {

    $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup_allcheck&quot; , &quot;'&quot; , &quot;).click(function() {
       $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup input[type=&quot;checkbox&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, $(this).is(&quot;:checked&quot;));
    });
                                
    $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup input[type=&quot;checkbox&quot;]&quot; , &quot;'&quot; , &quot;).click(function() {
        var notCheckedCount = $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup input[name=&quot;leaveList[chkSearchFilter][]&quot;]:not(:checked)&quot; , &quot;'&quot; , &quot;).length;

        $(&quot; , &quot;'&quot; , &quot;#leaveList_chkSearchFilter_checkboxgroup_allcheck&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, notCheckedCount === 0);
    });
});

 

                &quot;))]</value>
      <webElementGuid>7ff0406c-2eff-401d-868d-bfdecb0777d9</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
