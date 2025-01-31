use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66759447: FileFormat = FileFormat {
    id: 66_759_447,
    puid: "wikidata/66759447",
    name: "Microsoft Office Access Signed Packages",
    extensions: &["accdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
