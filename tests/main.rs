use std::str;
use folia;

const example: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<FoLiA xmlns="http://ilk.uvt.nl/folia" version="2.0" xml:id="example">
  <metadata>
      <annotations>
          <token-annotation set="https://raw.githubusercontent.com/LanguageMachines/uctodata/master/setdefinitions/tokconfig-eng.foliaset.ttl">
			 <annotator processor="p1" />
		  </token-annotation>
          <text-annotation>
			 <annotator processor="p1" />
          </text-annotation>
          <sentence-annotation>
			 <annotator processor="p1" />
          </sentence-annotation>
          <paragraph-annotation>
			 <annotator processor="p1" />
          </paragraph-annotation>
      </annotations>
      <provenance>
         <processor xml:id="p1" name="proycon" type="manual" />
      </provenance>
  </metadata>
  <text xml:id="example.text">
    <p xml:id="example.p.1">
      <s xml:id="example.p.1.s.1">
         <w xml:id="example.p.1.s.1.w.1" class="WORD">
            <t>Hello</t>
         </w>
         <w xml:id="example.p.1.s.1.w.2" class="WORD" space="no">
            <t>World</t>
         </w>
         <w xml:id="example.p.1.s.1.w.3" class="PUNCTUATION">
            <t>!</t>
         </w>
      </s>
      <s xml:id="example.p.1.s.2">
         <w xml:id="example.p.1.s.2.w.1" class="WORD">
            <t>This</t>
         </w>
         <w xml:id="example.p.1.s.2.w.2" class="WORD">
            <t>is</t>
         </w>
         <w xml:id="example.p.1.s.2.w.3" class="WORD">
            <t>an</t>
         </w>
         <w xml:id="example.p.1.s.2.w.4" class="WORD" space="no">
            <t>example</t>
         </w>
         <w xml:id="example.p.1.s.2.w.5" class="PUNCTUATION">
            <t>.</t>
         </w>
      </s>
    </p>
  </text>
</FoLiA>"#;

#[test]
fn instantiate() {
    if let Ok(doc) = folia::Document::new("example", folia::BodyType::Text) {
        assert_eq!(doc.id(), "example");
    } else {
        assert!(false);
    }
}

#[test]
fn append() {
    if let Ok(doc) = folia::Document::new("example", folia::BodyType::Text) {
        let attribs = vec![ folia::Attribute::Id("s.1".to_string()) ];
        doc.body.unwrap().append(folia::ElementType::Sentence, Some(attribs), None  );
    } else {
        assert!(false);
    }
}

#[test]
fn parse() {
    match folia::Document::from_str(str::from_utf8(example).expect("invalid utf-8 in example")) {
        Ok(doc) => {
            assert_eq!(doc.id(), "example");
        }
        Err(err) => {
            println!("{}", err);
            assert!(false);
        }
    }
}
