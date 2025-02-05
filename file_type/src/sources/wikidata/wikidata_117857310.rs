use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117857310: FileFormat = FileFormat {
    id: 117_857_310,
    source_type: SourceType::Wikidata,
    name: "IBM Storyboard PIC file",
    extensions: &["sbp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
