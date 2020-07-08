/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    bool find(TreeNode* root,int sum,int target)
    {
        if(!root)return false;
        sum+=root->val;
        if(root->left==NULL&&root->right==NULL)
        {
            if(sum==target)
            {
                return true;
            }
            return false;
        }
        else
        {
            return find(root->right,sum,target)||find(root->left,sum,target);
        }
        return false;
    }
    bool hasPathSum(TreeNode* root, int sum) {
        if (root==NULL){
            return false;
        }else{
            return find(root,0,sum);
        }
    }
};