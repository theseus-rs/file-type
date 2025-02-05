use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111342190: FileFormat = FileFormat {
    id: 111_342_190,
    source_type: SourceType::Wikidata,
    name: "Avalon sample",
    extensions: &["smp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
