use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131626588: FileFormat = FileFormat {
    id: 131_626_588,
    source_type: SourceType::Wikidata,
    name: "FLUENT CFF file format",
    extensions: &["dat.h5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
