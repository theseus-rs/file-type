use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83795552: FileFormat = FileFormat {
    id: 83_795_552,
    source_type: SourceType::Wikidata,
    name: "ZFO (Proof of Delivery) File",
    extensions: &["zfo"],
    media_types: &["application/vnd.software602.filler.form-xml-zip"],
    signatures: &[],
    related_formats: &[],
};
