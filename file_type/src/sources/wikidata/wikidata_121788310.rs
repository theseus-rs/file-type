use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121788310: FileFormat = FileFormat {
    id: 121_788_310,
    puid: "wikidata/121788310",
    name: "Microsoft Powerpoint for Macintosh 3",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-PowerPoint"],
    internal_signatures: &[],
    related_formats: &[],
};
