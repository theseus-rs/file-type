use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128123256: FileFormat = FileFormat {
    id: 128_123_256,
    source_type: SourceType::Wikidata,
    name: "Stylus file format",
    extensions: &["styl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
