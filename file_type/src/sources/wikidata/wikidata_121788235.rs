use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121788235: FileFormat = FileFormat {
    id: 121_788_235,
    puid: "wikidata/121788235",
    name: "Microsoft Powerpoint for Macintosh 2",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-PowerPoint"],
    internal_signatures: &[],
    related_formats: &[],
};
