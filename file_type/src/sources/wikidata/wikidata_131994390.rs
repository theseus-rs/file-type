use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131994390: FileFormat = FileFormat {
    id: 131_994_390,
    source_type: SourceType::Wikidata,
    name: "Web Extracted Text",
    extensions: &["wet"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
