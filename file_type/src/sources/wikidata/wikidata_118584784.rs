use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118584784: FileFormat = FileFormat {
    id: 118_584_784,
    source_type: SourceType::Wikidata,
    name: "Cakewalk Bundle",
    extensions: &["cwb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
