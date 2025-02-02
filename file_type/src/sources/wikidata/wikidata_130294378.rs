use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130294378: FileFormat = FileFormat {
    id: 130_294_378,
    source_type: SourceType::Wikidata,
    name: "MIPS file format",
    extensions: &["mips"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
