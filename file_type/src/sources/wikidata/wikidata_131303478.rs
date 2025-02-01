use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131303478: FileFormat = FileFormat {
    id: 131_303_478,
    puid: "wikidata/131303478",
    name: "TiddlerFile",
    extensions: &["tid", "tid"],
    media_types: &["application/x-tiddler", "text/vnd.tiddlywiki"],
    internal_signatures: &[],
    related_formats: &[],
};
