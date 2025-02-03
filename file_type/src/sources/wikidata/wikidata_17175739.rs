use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17175739: FileFormat = FileFormat {
    id: 17_175_739,
    source_type: SourceType::Wikidata,
    name: "comic book archive, RAR container",
    extensions: &["cbr"],
    media_types: &["application/vnd.comicbook-rar"],
    internal_signatures: &[],
    related_formats: &[],
};
