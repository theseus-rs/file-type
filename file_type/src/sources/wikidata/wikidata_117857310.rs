use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117857310: FileFormat = FileFormat {
    id: 117_857_310,
    source_type: SourceType::Wikidata,
    name: "IBM Storyboard PIC file",
    extensions: &["sbp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
