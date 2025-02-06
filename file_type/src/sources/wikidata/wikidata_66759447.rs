use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66759447: FileFormat = FileFormat {
    id: 66_759_447,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Access Signed Packages",
    extensions: &["accdc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
