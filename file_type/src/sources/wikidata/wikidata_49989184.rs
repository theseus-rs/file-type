use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49989184: FileFormat = FileFormat {
    id: 49_989_184,
    source_type: SourceType::Wikidata,
    name: "Macro Enabled Microsoft Powerpoint",
    extensions: &["pptm"],
    media_types: &["application/vnd.openxmlformats"],
    signatures: &[],
    related_formats: &[],
};
