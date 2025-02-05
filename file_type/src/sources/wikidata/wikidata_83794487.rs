use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83794487: FileFormat = FileFormat {
    id: 83_794_487,
    source_type: SourceType::Wikidata,
    name: "ZFO (Form) File",
    extensions: &["zfo"],
    media_types: &["application/vnd.software602.filler.form-xml-zip"],
    signatures: &[],
    related_formats: &[],
};
