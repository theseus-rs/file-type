use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61718355: FileFormat = FileFormat {
    id: 61_718_355,
    source_type: SourceType::Wikidata,
    name: "Microsoft PowerPoint for Macintosh, version 4",
    extensions: &["ppt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
