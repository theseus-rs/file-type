use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50419827: FileFormat = FileFormat {
    id: 50_419_827,
    source_type: SourceType::Wikidata,
    name: "Microsoft PRX File",
    extensions: &["prx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
