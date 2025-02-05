use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55758988: FileFormat = FileFormat {
    id: 55_758_988,
    source_type: SourceType::Wikidata,
    name: "Microsoft Program Database file format, version 2",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
