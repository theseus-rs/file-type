use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61752300: FileFormat = FileFormat {
    id: 61_752_300,
    puid: "wikidata/61752300",
    name: "Microsoft FrontPage file format",
    extensions: &["lck"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
