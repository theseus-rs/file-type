use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333099: FileFormat = FileFormat {
    id: 111_333_099,
    source_type: SourceType::Wikidata,
    name: "Korg PA1X/PA800/PA2X samples",
    extensions: &["pcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
