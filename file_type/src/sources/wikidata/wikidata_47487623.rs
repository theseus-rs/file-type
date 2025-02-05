use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47487623: FileFormat = FileFormat {
    id: 47_487_623,
    source_type: SourceType::Wikidata,
    name: "GEM Metafile",
    extensions: &["gem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
