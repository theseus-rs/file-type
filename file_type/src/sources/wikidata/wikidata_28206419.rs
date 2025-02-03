use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206419: FileFormat = FileFormat {
    id: 28_206_419,
    source_type: SourceType::Wikidata,
    name: "LuraWave JPEG-2000 Code Stream Format",
    extensions: &["jpc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
