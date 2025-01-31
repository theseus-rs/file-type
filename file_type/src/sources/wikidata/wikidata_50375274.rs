use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50375274: FileFormat = FileFormat {
    id: 50_375_274,
    puid: "wikidata/50375274",
    name: "Microsoft OneNote Package File",
    extensions: &["onepkg"],
    media_types: &["application/onenote"],
    internal_signatures: &[],
    related_formats: &[],
};
