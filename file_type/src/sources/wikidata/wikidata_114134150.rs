use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114134150: FileFormat = FileFormat {
    id: 114_134_150,
    source_type: SourceType::Wikidata,
    name: "MOPAC format",
    extensions: &["mop"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
