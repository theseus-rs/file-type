use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128123256: FileFormat = FileFormat {
    id: 128_123_256,
    source_type: SourceType::Wikidata,
    name: "Stylus file format",
    extensions: &["styl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
