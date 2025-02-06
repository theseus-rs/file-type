use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127327283: FileFormat = FileFormat {
    id: 127_327_283,
    source_type: SourceType::Wikidata,
    name: "Ada file",
    extensions: &["adb"],
    media_types: &["text/x-ada"],
    signatures: &[],
    related_formats: &[],
};
