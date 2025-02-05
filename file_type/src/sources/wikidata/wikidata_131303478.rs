use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131303478: FileFormat = FileFormat {
    id: 131_303_478,
    source_type: SourceType::Wikidata,
    name: "TiddlerFile",
    extensions: &["tid"],
    media_types: &["application/x-tiddler", "text/vnd.tiddlywiki"],
    signatures: &[],
    related_formats: &[],
};
