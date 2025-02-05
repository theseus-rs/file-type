use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83795336: FileFormat = FileFormat {
    id: 83_795_336,
    source_type: SourceType::Wikidata,
    name: "ZFO (Sent Message) File",
    extensions: &["zfo"],
    media_types: &["application/vnd.software602.filler.form-xml-zip"],
    signatures: &[],
    related_formats: &[],
};
