use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81192187: FileFormat = FileFormat {
    id: 81_192_187,
    source_type: SourceType::Wikidata,
    name: "Microsoft Border art",
    extensions: &["bdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
