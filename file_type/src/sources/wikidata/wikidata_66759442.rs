use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66759442: FileFormat = FileFormat {
    id: 66_759_442,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database Templates",
    extensions: &["accdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
