use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114238261: FileFormat = FileFormat {
    id: 114_238_261,
    source_type: SourceType::Wikidata,
    name: "Streaming Audio Metafile",
    extensions: &["lam"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
