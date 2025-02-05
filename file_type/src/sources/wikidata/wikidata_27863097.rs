use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863097: FileFormat = FileFormat {
    id: 27_863_097,
    source_type: SourceType::Wikidata,
    name: "3GPP file format",
    extensions: &["3gp"],
    media_types: &["audio/3gpp", "video/3gpp"],
    signatures: &[],
    related_formats: &[],
};
