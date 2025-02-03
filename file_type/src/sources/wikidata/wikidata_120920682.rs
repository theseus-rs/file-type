use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120920682: FileFormat = FileFormat {
    id: 120_920_682,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2004 backup data",
    extensions: &["m12"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
