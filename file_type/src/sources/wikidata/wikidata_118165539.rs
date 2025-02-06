use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118165539: FileFormat = FileFormat {
    id: 118_165_539,
    source_type: SourceType::Wikidata,
    name: "FotoFinish Image Format",
    extensions: &["sph"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
