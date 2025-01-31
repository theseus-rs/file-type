use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109673241: FileFormat = FileFormat {
    id: 109_673_241,
    puid: "wikidata/109673241",
    name: "MS Outlook Express Email Index",
    extensions: &["idx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
