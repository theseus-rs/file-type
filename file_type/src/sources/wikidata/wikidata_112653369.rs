use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112653369: FileFormat = FileFormat {
    id: 112_653_369,
    source_type: SourceType::Wikidata,
    name: "Astound Draw backup file",
    extensions: &["ad~"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
