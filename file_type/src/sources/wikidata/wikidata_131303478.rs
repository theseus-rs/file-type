use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131303478: FileFormat = FileFormat {
    id: 131_303_478,
    source_type: SourceType::Wikidata,
    name: "TiddlerFile",
    extensions: &["tid"],
    media_types: &["application/x-tiddler", "text/vnd.tiddlywiki"],
    internal_signatures: &[],
    related_formats: &[],
};
