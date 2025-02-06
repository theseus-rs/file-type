use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113292166: FileFormat = FileFormat {
    id: 113_292_166,
    source_type: SourceType::Wikidata,
    name: "Macintosh Sound Resource",
    extensions: &["snd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
